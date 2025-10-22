package com.secure_ticket.Model;

import java.time.LocalDate;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;

@Entity
public class Event {
    @Id
    @GeneratedValue(strategy= GenerationType.IDENTITY)
    private long id;

    @Column(name = "NAME", nullable = false)
    private String name;

    @Column(name = "ARTIST", nullable = false)
    private String artist;

    @Column(name = "DATE", nullable = false)
    private LocalDate date;

    @Column(name = "LOCATION", nullable = false)
    private String location;

}
