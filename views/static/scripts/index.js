"use strict";
const form_element = document.querySelector("form#input_form");
form_element.addEventListener("submit", submitHandler);
function submitHandler(event) {
    event.preventDefault();
    const forms = document.querySelector("form#input_form");
    const valid = forms.reportValidity();
    if (!valid) {
        return;
    }
    const result = getFormJSON(forms);
    fetch('/submitPresensi', {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(result)
    })
        .then((response) => {
        if (response.ok) {
            window.location.href = `/success?source=Presensi&nama=${result.nama}`;
        }
    });
}
function getFormJSON(form) {
    const data = new FormData(form);
    const retval = {
        nama: data.get("nama"),
        tanggal: data.get("tanggal"),
        status_hadir: data.get("status_hadir"),
        alasan: data.get("alasan")
    };
    return retval;
}
;
