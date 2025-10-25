package com.secure_ticket.Model;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;


@Entity
@Table(name = "request")
public class Request {
    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private long id;

    @Column(name = "DESCRIPTION", nullable = false)
    private String description;

    @Column(name = "STATUS", nullable = false)
    private String status;

    // Usaremos el ID primitivo del usuario (sin relación JPA)
    @Column(name = "USER_ID", nullable = false)
    private long userId; 

    @Column(name = "EVENT_NAME", nullable = true)
    private String eventName;

    @Column(name = "TYPE", nullable = false)
    private String type;
    public Request() {
        this.status = "Pendiente";
    }

    public Request(String description, long userId, String eventName, String type) {
        this.description = description;
        this.status = "Pendiente";
        this.userId = userId;
        this.eventName = eventName;
        this.type = type;
    }

    public long getId() {
        return id;
    }

    public String getDescription() {
        return description;
    }
    public void setDescription(String description) {
        this.description = description;
    }
    
    public String getStatus() {
        return status;
    }
    public void setStatus(String status) {
        this.status = status;
    }
    
    public long getUserId() {
        return userId;
    }
    public void setUserId(long userId) {
        this.userId = userId;
    }
    
    public String getEventName() {
        return eventName;
    }
    public void setEventName(String eventName) {
        this.eventName = eventName;
    }
    
    public String getType() {
        return type;
    }
    public void setType(String type) {
        this.type = type;
    }
}