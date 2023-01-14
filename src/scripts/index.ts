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

	let earlyreturn = false;

	if (result.status_hadir==="absen" && result.alasan === "") {
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
			if (response.ok){
				window.location.href = `/success?source=Presensi&nama=${result.nama}`
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

function highlight(id: string) {
	const elem = document.querySelector(`div#${id}`) as HTMLDivElement;
	elem.style.borderColor = "red";
	elem.scrollIntoView();
}

