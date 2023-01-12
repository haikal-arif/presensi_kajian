
function submitHandler (event: SubmitEvent): void {
    event.preventDefault();
    const forms = (document.querySelector("form#input_form") as HTMLFormElement);
    const valid = forms.reportValidity();

	if (!valid) {
		return
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
			if (response.ok){
				window.location.href = '/success'
			}
	})
}

function getFormJSON (form: HTMLFormElement) : JSON {
    const data = new FormData(form);
	
    let retval: any = {
        "nama": data.get("nama"),
        "tanggal": data.get("tanggal"),
        "status_hadir": data.get("status_hadir"),
        "alasan": data.get("alasan")
    };

    return retval;
    
  };

