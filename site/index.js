const js = import("./node_modules/@andriiant/apr_wasm/apr_wasm.js");
let result = document.getElementById('hi')


let amount = document.getElementById('amount')
let apr = document.getElementById('apr')
let redelegate = document.getElementById('redelegate')
let period = document.getElementById('period')
let button = document.getElementById('calculate')

button.onclick = () => {
  js.then(js => {
    result.textContent = ('After ' +period.value+ ' days, you get: ' +js.apr(amount.value, apr.value / 100, redelegate.value, period.value).toFixed(5)+ ' tokens.')

  });
}





// getinfo().then()