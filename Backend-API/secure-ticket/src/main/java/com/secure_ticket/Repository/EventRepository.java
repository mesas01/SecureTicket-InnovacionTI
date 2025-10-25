package com.secure_ticket.Repository;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import com.secure_ticket.Model.Event;

@Repository
public interface EventRepository extends JpaRepository<Event, Long>{

}
