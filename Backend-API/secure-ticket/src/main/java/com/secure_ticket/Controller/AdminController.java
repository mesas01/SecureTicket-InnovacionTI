package com.secure_ticket.Controller;

import java.io.IOException;
import java.util.List;
import java.util.Map;
import java.util.Optional;
import java.util.stream.Collectors;

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

import com.secure_ticket.DTO.RequestDTO;
import com.secure_ticket.Model.Event;
import com.secure_ticket.Model.Request;
import com.secure_ticket.Model.User; 
import com.secure_ticket.Repository.EventRepository;
import com.secure_ticket.Repository.RequestRepository; 
import com.secure_ticket.Repository.UserRepository;
import com.secure_ticket.Service.CloudinaryService;
import com.secure_ticket.Service.UserService;

@Controller
@RequestMapping("/admin") 
public class AdminController {


    private final UserService userService; 
    private final EventRepository eventRepository; 
    private final CloudinaryService cloudinaryService;
    private final RequestRepository requestRepository; 
    private final UserRepository userRepository;

    public AdminController(
    UserService userService, 
    EventRepository eventRepository, 
    CloudinaryService cloudinaryService,
    RequestRepository requestRepository,
    UserRepository userRepository

    ) {
        this.userService = userService;
        this.eventRepository = eventRepository;
        this.cloudinaryService = cloudinaryService;
        this.requestRepository = requestRepository;
        this.userRepository = userRepository;
    }


    @GetMapping
    public String showAdminDashboard(Model model) {
        
        model.addAttribute("allUsers", userService.findAllAdminDTOs()); 
        
        List<Event> allEvents = eventRepository.findAll();
        model.addAttribute("allEvents", allEvents);
        model.addAttribute("newEvent", new Event());
        
        List<Request> requests = requestRepository.findAll();
        List<RequestDTO> requestDtos = requests.stream().map(request -> {
            String username = userRepository.findById(request.getUserId())
                                            .map(User::getUsername)
                                            .orElse("[Usuario Eliminado]");
            return new RequestDTO(
                request.getId(),
                username, 
                request.getType(),
                request.getDescription(),
                request.getEventName(),
                request.getStatus()
            );
        }).collect(Collectors.toList());
        
        model.addAttribute("requestDtos", requestDtos);

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

    @PostMapping("/request/change-status/{id}")
    public String changeRequestStatus(@PathVariable Long id, RedirectAttributes redirectAttributes) {
        Optional<Request> requestOpt = requestRepository.findById(id);

        if (requestOpt.isPresent()) {
            Request request = requestOpt.get();
            
            if ("Pendiente".equals(request.getStatus())) {
                request.setStatus("En Proceso");
            } else if ("En Proceso".equals(request.getStatus())) {
                request.setStatus("Resuelto");
            } else {
                request.setStatus("Pendiente"); 
            }
            
            requestRepository.save(request);
            redirectAttributes.addFlashAttribute("successMessage", "Estado de la solicitud ID " + id + " actualizado a: " + request.getStatus());
        } else {
            redirectAttributes.addFlashAttribute("errorMessage", "Solicitud no encontrada.");
        }

        return "redirect:/admin";
    }
}