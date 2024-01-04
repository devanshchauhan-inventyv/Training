import { generatePascalTriangleAndPrimes } from "./logic.mjs";

// Function to generate Pascal's Triangle in HTML div elements and highlight primes
function generatePascalsTriangleHTML(triangleData) {
  const triangle = triangleData.pascalTriangle;
  const primes = triangleData.primes;
  const diva = document.getElementById('container');
  const errorbox = document.getElementById('error-box');
  const container = document.createElement('div');
  container.style.textAlign = 'center';
  const notFoundPrimes = [];

  for (let i = 0; i < triangle.length; i++) {
    const row = triangle[i];
    const rowDiv = document.createElement('div');
    rowDiv.style.display = 'flex';
    rowDiv.style.justifyContent = 'center';

    for (let j = 0; j < row.length; j++) {
      const numberSpan = document.createElement('span');
      numberSpan.style.padding = '10px';

      // Check if the number is a prime and color it accordingly
      if (primes.includes(row[j])) {
        numberSpan.style.color = 'black';
        numberSpan.style.textDecoration = 'underline';
        numberSpan.style.backgroundColor = 'yellow';
      }

      numberSpan.textContent = row[j];
      rowDiv.appendChild(numberSpan);
    }
    container.appendChild(rowDiv);
  }
  diva.appendChild(container)

  // Find prime numbers not present in Pascal's Triangle
  for (let i = 0; i < primes.length; i++) {
    if (!triangle.flat().includes(primes[i])) {
      notFoundPrimes.push(primes[i]);
    }
  }

  // Display prime numbers not found in Pascal's Triangle
  if (notFoundPrimes.length > 0) {
    const errorDiv = document.createElement('div');
    errorDiv.textContent = `
    Prime numbers not found in Pascal's Triangle: ${notFoundPrimes.join(', ')}`;
    errorDiv.textContent +=`
    Error Ratio is ${notFoundPrimes.length}`;
    errorDiv.style.marginTop = '20px';
    errorDiv.style.textAlign = 'center';
    errorbox.appendChild(errorDiv)
  }
}

const btn = document.getElementById("show")

const main= () => {
  const inputNumber = prompt("enter a number");
  const result = generatePascalTriangleAndPrimes(inputNumber);
  
  // Clear existing content in the container
  const container = document.getElementById('container');
  container.innerHTML = ''; // Clear the content
  // Generate Pascal's Triangle in HTML div elements and highlight primes
  console.time("start")
  generatePascalsTriangleHTML(result);
  console.timeEnd("start")
};

main();