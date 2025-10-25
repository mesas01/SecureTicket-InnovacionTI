function confirmDeletion(buttonElement) {
    const username = buttonElement.getAttribute('data-username');
    return confirm('¿Seguro que quieres eliminar al usuario ' + username + '?');
}

function confirmEventDeletion(buttonElement) {
    const eventTitle = buttonElement.getAttribute('data-eventtitle');
    return confirm('¿Seguro que quieres eliminar el evento ' + eventTitle + '?');
}