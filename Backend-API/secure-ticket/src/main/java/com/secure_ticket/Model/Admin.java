package com.secure_ticket.Model;

import jakarta.persistence.DiscriminatorValue;
import jakarta.persistence.Entity;
import jakarta.persistence.Table;

@Entity
@Table(name = "admin")
@DiscriminatorValue("ADMIN")
public class Admin extends User {
    public Admin() {
        super();
    }


}
