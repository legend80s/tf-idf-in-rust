{
  const input = document.querySelector('#query');
  const output = document.querySelector('#output');

  document.querySelector('#submit').addEventListener('submit', (event) => {
    event.preventDefault();

    output.innerHTML = '<p>Loading...</p>';
    postJSON(input.value);
  });

  // postJSON();
  async function postJSON(data) {
    try {
      const response = await fetch('/api/search', {
        method: 'POST',
        headers: {
          'Content-Type': 'text/plain',
        },
        body: data,
        // body: 'glsl function for linearly interpolation',
        // body: 'bind texture, to buffer.',
      });

      const result = await response.text();
      console.log('Success:', result);

      const ps = result.split('\n').map((str) => {
        const p = document.createElement('p');
        p.textContent = str;

        return p;
      });

      output.innerHTML = '';
      output.append(...ps);
    } catch (error) {
      console.error('Error:', error);
    }
  }
}
