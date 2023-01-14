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
    let earlyreturn = false;
    if (result.status_hadir === "absen" && result.alasan === "") {
        highlight("reason");
        earlyreturn = true;
    }
    if (result.status_hadir === null) {
        highlight("presence");
        earlyreturn = true;
    }
    if (result.tanggal === "") {
        highlight("date");
        earlyreturn = true;
    }
    if (result.nama === null) {
        highlight("name");
        earlyreturn = true;
    }
    if (earlyreturn) {
        return;
    }
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
        else {
            return response.text();
        }
    })
        .then(data => {
        if (data === undefined) {
            return;
        }
        alert(data);
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
function highlight(id) {
    const elem = document.querySelector(`div#${id}`);
    elem.style.borderColor = "red";
    elem.scrollIntoView();
}
