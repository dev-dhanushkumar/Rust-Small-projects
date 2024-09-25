import init, {calculate_tax} from "../pkg/tax_webassembly.js"

async function run() {
    await init();
    function calculateTax() {
        const income = document.getElementById("income").value;
        const tax = calculate_tax(income);
        document.getElementById("result").innerText = `Tax:$${tax.toFixed(2)}`;
    }

    document.getElementById("calculateButton").addEventListener("click", calculateTax);
}

run();