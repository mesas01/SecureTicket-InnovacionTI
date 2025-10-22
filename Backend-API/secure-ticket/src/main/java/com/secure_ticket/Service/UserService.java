// src/main/java/com/secure_ticket/Service/UserService.java

package com.secure_ticket.Service;

import java.util.Optional;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.dao.DataIntegrityViolationException;
import org.springframework.stereotype.Service;

import com.secure_ticket.Model.User;
import com.secure_ticket.Repository.UserRepository;

@Service
public class UserService {

    @Autowired
    private UserRepository userRepository;

    public User registerNewUser(User user) throws DataIntegrityViolationException {
        
        return userRepository.save(user);
    }

    public User validateCredentials(String loginIdentifier, String password) {
    Optional<User> userOptional = userRepository.findByUsernameOrEmail(loginIdentifier, loginIdentifier);
        
    if (userOptional.isPresent()) {
        User user = userOptional.get();
            if (user.getPassword().equals(password)) {
            return user;
            }
        }
    return null;
    }
}