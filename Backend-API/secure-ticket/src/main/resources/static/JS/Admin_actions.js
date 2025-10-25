function confirmDeletion(buttonElement) {
    // Función para confirmar la eliminación de usuarios
    const username = buttonElement.getAttribute('data-username');
    return confirm('¿Seguro que quieres eliminar al usuario ' + username + '?');
}

function confirmEventDeletion(buttonElement) {
    // Función para confirmar la eliminación de eventos
    const eventTitle = buttonElement.getAttribute('data-eventtitle');
    return confirm('¿Seguro que quieres eliminar el evento ' + eventTitle + '?');
}