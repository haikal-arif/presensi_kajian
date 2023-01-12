const form_registrasi = document.querySelector("form#form_registrasi") as HTMLFormElement;
form_registrasi.addEventListener("submit", submitHandlerFactory(form_registrasi));


function submitHandlerFactory(element: HTMLFormElement) {


    const submitFormRegistrasi = (event: SubmitEvent) => {
        event.preventDefault();

        if (!element.reportValidity())
            return;
        
        const result = {
            "nama" : new FormData(element).get("nama")
        }

        fetch('/registerSantri', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(result)
        })
        .then((response) => {
                if (response.ok){
                    window.location.href = '/success'
                }
        })
        
    }

    return submitFormRegistrasi
}