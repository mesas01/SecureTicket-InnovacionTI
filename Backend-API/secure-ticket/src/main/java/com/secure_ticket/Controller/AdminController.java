// src/main/java/com/secure_ticket/Controller/AdminController.java

package com.secure_ticket.Controller;

import java.io.IOException;
import java.util.List;
import java.util.Map;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.multipart.MultipartFile;
import org.springframework.web.servlet.mvc.support.RedirectAttributes;

import com.secure_ticket.Model.Event;
import com.secure_ticket.Repository.EventRepository;
import com.secure_ticket.Service.CloudinaryService;
import com.secure_ticket.Service.UserService;

import jakarta.servlet.http.HttpSession;

@Controller
@RequestMapping("/admin") // Todos los métodos usan el prefijo /admin
public class AdminController {


    private final UserService userService; 
    private final EventRepository eventRepository; 
    private final CloudinaryService cloudinaryService;

    public AdminController(
    UserService userService, 
    EventRepository eventRepository, 
    CloudinaryService cloudinaryService
    ) {
        this.userService = userService;
        this.eventRepository = eventRepository;
        this.cloudinaryService = cloudinaryService;
    }

    
    @GetMapping
    public String showAdminDashboard(Model model, HttpSession session) {
        
        if (session.getAttribute("currentUser") == null) {
            return "redirect:/login"; 
        }

        model.addAttribute("allUsers", userService.findAllAdminDTOs()); 
        
        List<Event> allEvents = eventRepository.findAll();
        model.addAttribute("allEvents", allEvents);
        model.addAttribute("newEvent", new Event());
        
        return "Admin.html";
    }

    @PostMapping("/add-event")
    public String addEvent(@ModelAttribute("newEvent") Event event, @RequestParam("imageFile") MultipartFile imageFile, RedirectAttributes redirectAttributes) {
        if(imageFile.isEmpty()){
            redirectAttributes.addFlashAttribute("error", "Image file is required.");
            return "redirect:/admin";
        }

        try{
            Map<String, Object> uploadResult = cloudinaryService.upload(imageFile);
            String imageUrl = (String) uploadResult.get("secure_url");
            event.setImageUrl(imageUrl);

            eventRepository.save(event);
            redirectAttributes.addFlashAttribute("success", "Evento agregado exitosamente.");
        } catch (IOException e) {
            redirectAttributes.addFlashAttribute("error", "No se pudo cargar la imagen. Error: " + e.getMessage());
        }
        
        return "redirect:/admin";
    }
    
    @PostMapping("/event/delete/{id}")
    public String deleteEvent(@PathVariable Long id) {
        eventRepository.deleteById(id);
        return "redirect:/admin";
    }

    @PostMapping("/user/delete/{id}")
    public String deleteUser(@PathVariable Long id) {
        userService.deleteUser(id);
        return "redirect:/admin";
    }
}