function toggleEventNameField() {
    var type = document.getElementById('type').value;
    var eventNameGroup = document.getElementById('eventNameGroup');
    if (type === 'Evento nuevo') {
        eventNameGroup.style.display = 'block';
        document.getElementById('eventName').setAttribute('required', 'required');
    } else {
        eventNameGroup.style.display = 'none';
        document.getElementById('eventName').removeAttribute('required');
    }
}
        
document.addEventListener('DOMContentLoaded', toggleEventNameField);