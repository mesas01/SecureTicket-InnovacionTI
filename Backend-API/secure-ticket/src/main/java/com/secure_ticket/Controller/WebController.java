package com.secure_ticket.Controller;

import java.util.List;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.dao.DataIntegrityViolationException;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.servlet.mvc.support.RedirectAttributes;

import com.secure_ticket.Model.Admin;
import com.secure_ticket.Model.Request;
import com.secure_ticket.Model.User;
import com.secure_ticket.Repository.EventRepository;
import com.secure_ticket.Repository.RequestRepository;
import com.secure_ticket.Repository.UserRepository; 
import com.secure_ticket.Service.UserService;

import jakarta.servlet.http.HttpSession;


@Controller
public class WebController {
    @Autowired
    private EventRepository eventRepository;

    @Autowired
    private UserService userService;
    
    @Autowired
    private RequestRepository requestRepository;

    @Autowired
    private UserRepository userRepository;
    
    @GetMapping("/")
    public String showMainPage(Model model) {
        model.addAttribute("events", eventRepository.findAll());
        return "MainPage.html";
    }

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
            session.setAttribute("currentUserId", authenticatedUser.getId()); 
            
            if (authenticatedUser instanceof Admin) { 
            return "redirect:/admin";
        } else {
            return "redirect:/";
        }
            
        } else {
            model.addAttribute("error", "Nombre de usuario, correo electrónico o contraseña incorrectos.");
            return "Login.html";
        }
    }

    @GetMapping("/logout")
    public String logout(HttpSession session) {
        session.removeAttribute("currentUser");
        session.removeAttribute("currentUserId");
        session.invalidate();
        
        return "redirect:/";
    }
    
    @GetMapping("/support")
    public String showSupportForm(Model model, HttpSession session) {
        if (session.getAttribute("currentUser") == null) {
        return "redirect:/login";
        }
        Long currentUserId = (Long) session.getAttribute("currentUserId");

        List<Request> userRequests = requestRepository.findByUserId(currentUserId);
        model.addAttribute("request", new Request());
        model.addAttribute("Requests", userRequests);

        return "Request.html";
    }

    @PostMapping("/support")
    public String submitSupportRequest(@ModelAttribute("request") Request request, HttpSession session, RedirectAttributes redirectAttributes) {
        Long userId = (Long) session.getAttribute("currentUserId");

        if (userId == null) {
            redirectAttributes.addFlashAttribute("errorMessage", "Debe iniciar sesión para enviar una solicitud.");
            return "redirect:/login";
        }
        
        try {
            request.setUserId(userId);
            request.setStatus("Pendiente");
            
            
            if (request.getEventName() == null || request.getEventName().isEmpty()) {
                request.setEventName("N/A");
            }
            requestRepository.save(request);

            redirectAttributes.addFlashAttribute("successMessage", "✅ Solicitud enviada exitosamente. Pronto será revisada.");
        } catch (Exception e) {
            redirectAttributes.addFlashAttribute("errorMessage", "❌ Error al enviar la solicitud: " + e.getMessage());
        }
        
        return "redirect:/support";
    }
}