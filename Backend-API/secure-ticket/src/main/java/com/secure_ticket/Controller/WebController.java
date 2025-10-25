package com.secure_ticket.Controller;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.dao.DataIntegrityViolationException;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestParam;

import com.secure_ticket.Model.Admin;
import com.secure_ticket.Model.User;
import com.secure_ticket.Repository.EventRepository;
import com.secure_ticket.Service.UserService;

import jakarta.servlet.http.HttpSession;




@Controller
public class WebController {
    @Autowired
    private EventRepository eventRepository;

    @GetMapping("/")
    public String showMainPage(Model model) {
        model.addAttribute("events", eventRepository.findAll());
        return "MainPage.html";
    }

    @Autowired
    private UserService userService;
    
    @GetMapping("/register")
    public String showRegisterPage(Model model) {
        model.addAttribute("user", new User());
        return "Register.html";
    }

    @PostMapping("/register")
    public String registerUser(@ModelAttribute("user") User user, Model model) {
        
        try {
            userService.registerNewUser(user);

            model.addAttribute("successMessage", "✅ ¡Registro exitoso!");
            return "redirect:/login";

        } catch (DataIntegrityViolationException e) {
            
            String errorMessage = "❌ Error: El nombre de usuario o el correo electrónico ya están registrados.";
            
            model.addAttribute("user", user);
            model.addAttribute("errorMessage", errorMessage);

            return "Register.html";
        }
    }

    @GetMapping("/login")
    public String showLoginForm() {
        return "Login.html";
    }

    @PostMapping("/login")
    public String processLogin(
        @RequestParam("username") String loginIdentifier,
        @RequestParam("password") String password,
        Model model,
        HttpSession session) {
        
        User authenticatedUser = userService.validateCredentials(loginIdentifier, password);
        
        if (authenticatedUser != null) {
            session.setAttribute("currentUser", authenticatedUser.getUsername());

            if (authenticatedUser instanceof Admin) { 
            return "redirect:/admin";
        } else {
            return "redirect:/";
        }
            
        } else {
            model.addAttribute("error", "Nombre de usuario, correo electrónico o contraseña incorrectos.");
            return "login.html";
        }
    }

    @GetMapping("/logout")
    public String logout(HttpSession session) {
        session.removeAttribute("currentUser");
        session.invalidate();
        
        return "redirect:/";
    }
}
