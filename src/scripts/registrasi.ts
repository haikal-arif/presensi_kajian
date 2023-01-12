const form_registrasi = document.querySelector("form#form_registrasi") as HTMLFormElement;
form_registrasi.addEventListener("submit", submitHandlerFactory(form_registrasi));

type RegistrationFormObj = {
	nama : string,
};

function submitHandlerFactory(element: HTMLFormElement) {


    const submitFormRegistrasi = (event: SubmitEvent) => {
        event.preventDefault();

        if (!element.reportValidity())
            return;
        
        const result: RegistrationFormObj = {
            nama : new FormData(element).get("nama") as string
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
                    window.location.href = `/success?source=Registrasi&nama=${result.nama}`
                }
        })
        
    }

    return submitFormRegistrasi
}