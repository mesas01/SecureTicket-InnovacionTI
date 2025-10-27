package com.secure_ticket.Service;

import org.springframework.mail.SimpleMailMessage;
import org.springframework.mail.javamail.JavaMailSender;
import org.springframework.stereotype.Service;

import com.secure_ticket.Model.Request;
import com.secure_ticket.Model.User;

@Service
public class EmailService {

    private final JavaMailSender mailSender;

    public EmailService(JavaMailSender mailSender) {
        this.mailSender = mailSender;
    }

    public void sendStatusUpdateEmail(String toEmail, String userName, Request request) {
        
        String subject = "Actualización de tu Solicitud de Soporte #" + request.getId();
        
        String body = String.format(
            "Hola %s,\n\n" +
            "Queremos informarte que el estado de tu solicitud de soporte ha sido actualizado:\n\n" +
            "  - Tipo de Solicitud: %s\n" +
            "  - Evento (si aplica): %s\n" +
            "  - Nuevo Estado: %s\n" +
            "  - Descripción original: \"%s\"\n\n" +
            "Gracias por tu paciencia.\n\n" +
            "Atentamente,\n" +
            "Equipo de Soporte Secure Ticket",
            userName,
            request.getType(),
            request.getEventName() != null ? request.getEventName() : "N/A",
            request.getStatus(),
            request.getDescription().substring(0, Math.min(request.getDescription().length(), 100)) + "..."
        );
        
        SimpleMailMessage message = new SimpleMailMessage();
        message.setTo(toEmail);
        message.setSubject(subject);
        message.setText(body);

        mailSender.send(message);
    }

    public void sendInformationEmail(User user, Request request, String adminMessage){
        String subject = "Se necesita más información - Solicitud de Soporte #" + request.getId();

        String body = String.format(
            "Hola %s,\n\n" +
            "Hemos revisado tu solicitud de soporte, pero necesitamos la siguiente información adicional para continuar con la resolución:\n\n" +
            "--- Mensaje del Administrador ---\n" +
            "%s\n" +
            "---------------------------------\n\n" +
            "  - Tu Solicitud Original: %s\n" +
            "  - Estado Actual: %s\n\n" +
            "Por favor, responde a este correo con la información solicitada.\n\n" +
            "Gracias,\n" +
            "Equipo de Soporte Secure Ticket",
            user.getUsername(),
            adminMessage,
            request.getDescription().substring(0, Math.min(request.getDescription().length(), 100)) + "...",
            request.getStatus()
        );

        SimpleMailMessage message = new SimpleMailMessage();
        message.setTo(user.getEmail());
        message.setSubject(subject);
        message.setText(body);

        mailSender.send(message);
    }

}