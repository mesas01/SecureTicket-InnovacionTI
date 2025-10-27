package com.secure_ticket.Model;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;


@Entity
public class Ticket {
    @Id
    @GeneratedValue(strategy= GenerationType.IDENTITY)
    private long id;

    @Column(name = "stellar_ticket_id", nullable = false, unique = true)
    private String stellarTicketId;

    @Column(name = "user_id", nullable = false)
    private long userId;

    @Column(name = "event_id", nullable = false)
    private long eventId;

    @Column(name = "price", nullable = false)
    private double price;
     
    public void setPrice(double price) {
        this.price = price;
    
    }
    public double getPrice() {
        return price;
    }

    public long getId() {
        return id;
    }
    public void setStellarTicketId(String stellarTicketId) {
        this.stellarTicketId = stellarTicketId;
    }
    public String getStellarTicketId() {
        return stellarTicketId;
    }
    public void setUserId(long userId) {
        this.userId = userId;
    }
    public long getUserId() {
        return userId;
    }
    public void setEventId(long eventId) {
        this.eventId = eventId;
    }
    public long getEventId() {
        return eventId;
    }

    public Ticket() {}

    public Ticket(String stellarTicketId, long userId, long eventId) {
        this.stellarTicketId = stellarTicketId;
        this.userId = userId;
        this.eventId = eventId;
    }
}
