function confirmDeletion(buttonElement) {
    const username = buttonElement.getAttribute('data-username');
    return confirm('¿Seguro que quieres eliminar al usuario ' + username + '?');
}

function confirmEventDeletion(buttonElement) {
    const eventTitle = buttonElement.getAttribute('data-eventtitle');
    return confirm('¿Seguro que quieres eliminar el evento ' + eventTitle + '?');
}

function showInfoRequestForm(requestId, username) {
    const message = prompt(`Escribe el mensaje que quieres enviar a ${username} (Solicitud #${requestId}):`);
    
    if (message !== null && message.trim() !== "") {
        const form = document.createElement('form');
        form.method = 'POST';
        form.action = `/admin/request/ask-info/${requestId}`;
        
        const input = document.createElement('input');
        input.type = 'hidden';
        input.name = 'adminComment';
        input.value = message;
        form.appendChild(input);

        document.body.appendChild(form);
        form.submit();
        
    } else if (message !== null) {
        alert("El mensaje no puede estar vacío.");
    }
}