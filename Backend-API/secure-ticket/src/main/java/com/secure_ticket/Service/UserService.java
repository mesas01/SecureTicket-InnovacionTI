package com.secure_ticket.Service;

import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.dao.DataIntegrityViolationException;
import org.springframework.stereotype.Service;

import com.secure_ticket.DTO.AdminDTO;
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

    public List<AdminDTO> findAllAdminDTOs() {
        return userRepository.findAll().stream()
            .map(this::convertToAdminDTO)
            .collect(Collectors.toList());
    }

    private AdminDTO convertToAdminDTO(User user) {
        String userType = user.getClass().getSimpleName();

        return new AdminDTO(user.getId(), user.getUsername(), user.getEmail(), userType, user.getDateOfBirth(), Long.parseLong(user.getPhone()), user.getAddress(), user.getCity());
    }

    public void deleteUser(Long id) {
        userRepository.deleteById(id);
    }

    public List<User> findAllUsers() {
        return userRepository.findAll();
    }
}