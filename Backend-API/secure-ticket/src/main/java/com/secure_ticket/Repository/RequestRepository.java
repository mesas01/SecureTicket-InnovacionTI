package com.secure_ticket.Repository;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

import com.secure_ticket.Model.Request;

import java.util.List;


@Repository
public interface RequestRepository extends JpaRepository<Request, Long> {
    List<Request> findByUserId(long userId);
}
