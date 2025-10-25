package com.secure_ticket.DTO;

import java.time.LocalDate;

public class AdminDTO {

    private Long id;
    private String username;
    private String email;
    private String userType;
    private LocalDate dateOfBirth;
    private long phone;
    private String address;
    private String city;


    public AdminDTO(Long id, String username, String email, String userType, LocalDate dateOfBirth, long phone, String address, String city) {
        this.id = id;
        this.username = username;
        this.email = email;
        this.userType = userType;
        this.dateOfBirth = dateOfBirth;
        this.phone = phone;
        this.address = address;
        this.city = city;
        this.userType = userType;
    }

    public AdminDTO() {}

    
    public Long getId() {
        return id;
    }
    public void setId(Long id) {
        this.id = id;
    }
    public String getUsername() {
        return username;
    }
    public void setUsername(String username) {
        this.username = username;
    }
    public String getEmail() {
        return email;
    }
    public void setEmail(String email) {
        this.email = email;
    }
    public String getUserType() {
        return userType;
    }
    public LocalDate getDateOfBirth() {
        return dateOfBirth;
    }
    public void setDateOfBirth(LocalDate dateOfBirth) {
        this.dateOfBirth = dateOfBirth;
    }
    public long getPhone() {
        return phone;
    }
    public void setPhone(long phone) {
        this.phone = phone;
    }
    public String getAddress() {
        return address;
    }
    public void setAddress(String address) {
        this.address = address;
    }
    public String getCity() {
        return city;
    }
    public void setCity(String city) {
        this.city = city;
    }


}