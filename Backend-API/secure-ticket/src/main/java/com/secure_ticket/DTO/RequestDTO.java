package com.secure_ticket.DTO;

public class RequestDTO {
    private long id;
    private String username;
    private String type; 
    private String description;
    private String eventName;
    private String status;

    public RequestDTO(long id, String username, String type, String description, String eventName, String status) {
        this.id = id;
        this.username = username;
        this.type = type;
        this.description = description;
        this.eventName = eventName;
        this.status = status;
    }

    public long getId() { return id; }
    public String getUsername() { return username; }
    public String getType() { return type; }
    public String getDescription() { return description; }
    public String getEventName() { return eventName; }
    public String getStatus() { return status; }
}