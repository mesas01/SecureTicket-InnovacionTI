#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Vec};
use soroban_sdk::token::Client as TokenClient;

// Base para cálculos de porcentaje
const PERCENT_BASE: i128 = 100;

#[derive(Clone)]
#[contracttype]
pub struct Ticket {
    pub id: u32,
    pub event_id: u32,
    pub owner: Address,
    pub price: i128,
    pub for_sale: bool,
    // para saber si este ticket ya fue revendido al menos 1 vez
    pub is_resale: bool,
    // para saber si el ticket ya fue escaneado y aprovado
    pub used: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    // Almacena un Ticket por su ID
    Ticket(u32),
    // Contador de tickets
    TicketCount,
    // La tiquetera ticketmaster/ tu boleta etc
    Organizer,
    // secure ticket, donde yo recibire comisiones
    Platform,
    // El token de pago usdc usdt etc
    Token,
    // % de comisión para el organizador en reventa
    OrgFee,
    // % de comisión para secure ticket
    PlatFee,
}

#[contract]
pub struct SecureTicketContract;

#[contractimpl]
impl SecureTicketContract {

    /// Inicializa el contrato.
    ///
    /// # Arguments
    /// * `organizer` - La dirección de la tiquetera que crea eventos.
    /// * `platform` - La dirección de tu secure ticket que recibe comisiones de reventa.
    /// * `token` - La dirección del contrato de token USDC usado para pagos.
    /// * `organizer_fee` - El porcentaje que gana la tiquetera
    /// * `platform_fee` - El porcentaje de ticket master
    pub fn initialize(
        env: Env,
        organizer: Address,
        platform: Address,
        token: Address,
        organizer_fee: i128,
        platform_fee: i128,
    ) {
        if env.storage().instance().has(&DataKey::Organizer) {
            panic!("already_init");
        }
        if organizer_fee + platform_fee >= PERCENT_BASE {
            // Las comisiones no pueden ser >= 100%
            panic!("fees_too_high");
        }
        if organizer_fee < 0 || platform_fee < 0 {
            panic!("negative_fees");
        }

        // Solo la tiquetera puede inicializar el contrato
        organizer.require_auth();

        env.storage().instance().set(&DataKey::Organizer, &organizer);
        env.storage().instance().set(&DataKey::Platform, &platform);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::OrgFee, &organizer_fee);
        env.storage().instance().set(&DataKey::PlatFee, &platform_fee);
        env.storage().instance().set(&DataKey::TicketCount, &0u32);
    }

    // Crea un nuevo ticket. Solo puede ser llamado por el 'organizer'.
    // El ticket se crea a nombre del organizador.
    pub fn create_ticket(env: Env, event_id: u32, price: i128) {
        if price < 0 {
            panic!("invalid_price");
        }

        let organizer: Address = env
            .storage()
            .instance()
            .get(&DataKey::Organizer)
            .expect("Not init");

        // Seguridad: Solo el organizador puede crear tickets
        organizer.require_auth();

        let mut ticket_count: u32 = env.storage().instance().get(&DataKey::TicketCount).unwrap_or(0);
        let ticket_id = ticket_count;

        let ticket = Ticket {
            id: ticket_id,
            event_id,
            owner: organizer.clone(),
            price,
            for_sale: false,
            // Es 'false' porque es nuevo
            is_resale: false,
            // Es 'false' porque es nuevo
            used: false,
        };

        env.storage().instance().set(&DataKey::Ticket(ticket_id), &ticket);
        ticket_count += 1;
        env.storage().instance().set(&DataKey::TicketCount, &ticket_count);
    }

    // Pone un ticket a la venta (para venta primaria o reventa).
    // Solo el dueño actual del ticket puede llamar esta función.
    pub fn list_ticket(env: Env, ticket_id: u32, new_price: i128) {
        let mut ticket: Ticket = Self::get_ticket(env.clone(), ticket_id);

        // Seguridad: Solo el dueño puede listar
        ticket.owner.require_auth();

        if ticket.for_sale {
            panic!("already_sale");
        }
        if ticket.used {
            // No se puede vender un ticket usado
            panic!("ticket_used");
        }
        if new_price < 0 {
            panic!("invalid_price");
        }

        ticket.price = new_price;
        ticket.for_sale = true;
        // is_resale no se modifica pq aquí. Se define en la compra.

        env.storage().instance().set(&DataKey::Ticket(ticket_id), &ticket);
    }

    // Cancela una venta listada.
    // Solo el dueño actual del ticket puede llamar esta función.
    pub fn cancel_sale(env: Env, ticket_id: u32) {
        let mut ticket: Ticket = Self::get_ticket(env.clone(), ticket_id);

        // Seguridad: Solo el dueño puede cancelar
        ticket.owner.require_auth();

        if !ticket.for_sale {
            panic!("not_sale");
        }

        ticket.for_sale = false;
        env.storage().instance().set(&DataKey::Ticket(ticket_id), &ticket);
    }

    // Compra un ticket que está a la venta.
    // El 'comprador' debe autorizar esta transacción.
    pub fn buy_ticket(env: Env, ticket_id: u32, buyer: Address) {
        let mut ticket = Self::get_ticket(env.clone(), ticket_id);

        // Seguridad: El comprador debe autorizar
        buyer.require_auth();

        if !ticket.for_sale {
            panic!("not_sale");
        }
        if ticket.used {
            panic!("ticket_used");
        }
        if ticket.owner == buyer {
            panic!("self_buy"); // No se puede auto comprar
        }

        let organizer: Address = env.storage().instance().get(&DataKey::Organizer).expect("Not init");
        let token: Address = env.storage().instance().get(&DataKey::Token).expect("Not init");
        let token_client = TokenClient::new(&env, &token);
        let price = ticket.price;
        let seller = ticket.owner.clone();

        if !ticket.is_resale {
            // si se compra el ticket por primera vez 100% del dinero va al organizador.
            token_client.transfer(&buyer, &organizer, &price);

            // se marca el ticket como 'reventa' para todas las futuras ventas
            ticket.is_resale = true;

        } else {
            // LÓGICA DE REVENTA (con comisiones)
            let platform: Address = env.storage().instance().get(&DataKey::Platform).expect("Not init");
            let org_fee_percent: i128 = env.storage().instance().get(&DataKey::OrgFee).expect("Not init");
            let plat_fee_percent: i128 = env.storage().instance().get(&DataKey::PlatFee).expect("Not init");

            // Calcular pagos
            let organizer_fee = price * org_fee_percent / PERCENT_BASE;
            let platform_fee = price * plat_fee_percent / PERCENT_BASE;
            let seller_amount = price - organizer_fee - platform_fee;

            // Distribuir fondos
            token_client.transfer(&buyer, &organizer, &organizer_fee);
            token_client.transfer(&buyer, &platform, &platform_fee);
            token_client.transfer(&buyer, &seller, &seller_amount);
        }

        // Actualizar estado del ticket
        ticket.owner = buyer;
        ticket.for_sale = false;

        env.storage().instance().set(&DataKey::Ticket(ticket_id), &ticket);
    }

    // Canjear el ticket en la puerta del evento.
    // Solo el dueño actual puede autorizar esto firmando la transacción con su wallet.
    // Esta acción es irreversible.
    pub fn redeem_ticket(env: Env, ticket_id: u32) {
        let mut ticket: Ticket = Self::get_ticket(env.clone(), ticket_id);

        // Seguridad: Solo el dueño puede canjear su ticket
        ticket.owner.require_auth();

        if ticket.used {
            panic!("already_used");
        }

        ticket.used = true;
        ticket.for_sale = false; // Seguridad: no se puede vender si ya se usó
        env.storage().instance().set(&DataKey::Ticket(ticket_id), &ticket);
    }

    // Funciones de Solo Lectura (Vistas)

    // Obtiene la información de un ticket por su ID.
    pub fn get_ticket(env: Env, ticket_id: u32) -> Ticket {
        env.storage()
            .instance()
            .get(&DataKey::Ticket(ticket_id))
            .expect("Ticket not found")
    }

    // Obtiene el dueño de un ticket por su ID.
    pub fn get_owner(env: Env, ticket_id: u32) -> Address {
        let ticket: Ticket = Self::get_ticket(env, ticket_id);
        ticket.owner
    }

    // Devuelve una lista de todos los tickets EN REVENTA que están a la venta.
    pub fn get_resale_tickets(env: Env) -> Vec<Ticket> {
        let ticket_count: u32 = env.storage().instance().get(&DataKey::TicketCount).unwrap_or(0);
        let mut resale_tickets = Vec::new(&env);

        for id in 0..ticket_count {
            if let Some(ticket) = env.storage().instance().get::<_, Ticket>(&DataKey::Ticket(id)) {
                // Debe estar a la venta Y ser una reventa (is_resale = true)
                if ticket.for_sale && ticket.is_resale {
                    resale_tickets.push_back(ticket);
                }
            }
        }
        resale_tickets
    }

    // Devuelve una lista de todos los tickets para un evento específico.
    pub fn get_event_tickets(env: Env, event_id: u32) -> Vec<Ticket> {
        let ticket_count: u32 = env.storage().instance().get(&DataKey::TicketCount).unwrap_or(0);
        let mut event_tickets = Vec::new(&env);

        for id in 0..ticket_count {
            if let Some(ticket) = env.storage().instance().get::<_, Ticket>(&DataKey::Ticket(id)) {
                if ticket.event_id == event_id {
                    event_tickets.push_back(ticket);
                }
            }
        }
        event_tickets
    }
}

#[cfg(test)]
mod test;