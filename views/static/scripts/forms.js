const form_element = document.getElementById("input_form");

const getFormJSON = (form) => {
    const data = new FormData(form);
	console.log(data)
    return Array.from(data.keys()).reduce((result, key) => {
      result[key] = data.get(key);
      return result;
    }, {});
  };

const handler = (event) => {
    event.preventDefault();
    const valid = form_element.reportValidity();

	if (!valid) {
		return
	}
	const result = getFormJSON(form_element);
	fetch('/submitPresensi', {
		method: 'POST',
		headers: {
			'Accept': 'application/json',
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(result)
	})
	.then(response => console.log(response))
}

form_element.addEventListener("submit", handler);


