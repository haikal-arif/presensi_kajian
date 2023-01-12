const form_element = document.querySelector("form#input_form") as HTMLFormElement;
form_element.addEventListener("submit", submitHandler);

function submitHandler (event: SubmitEvent): void {
    event.preventDefault();
    const forms = document.querySelector("form#input_form") as HTMLFormElement;
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
				window.location.href = `/success?source=Presensi&nama=${result.nama}`
			}
	})
}

type AttendanceFormObj = {
	nama : string,
	tanggal: string,
	status_hadir: string,
	alasan: string
};

function getFormJSON (form: HTMLFormElement) : AttendanceFormObj {
    const data = new FormData(form);
	
    const retval: AttendanceFormObj ={
        nama: data.get("nama") as string,
        tanggal: data.get("tanggal") as string,
        status_hadir: data.get("status_hadir") as string,
        alasan: data.get("alasan") as string
    };

    return retval;
    
};

