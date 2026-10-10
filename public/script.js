const form   = document.getElementById("calcForm");
const result = document.getElementById("result");
const button = document.getElementById("calculateBtn");

form.addEventListener("submit", async (event) => {
    event.preventDefault(); // предотвращаем отправку формы и перезагрузку страницы

    const a = Number(document.getElementById("a").value);
    const b = Number(document.getElementById("b").value);
    const operation = document.getElementById("operation").value;

    button.disabled = true;
    result.textContent = "Вычисление...";

    try {
        // Отправляем POST-запрос на сервер с данными для вычисления возвращает объект promise
        const response = await fetch("/api/calculate", {
            method: "POST",
            headers: { "Content-Type": "application/json"},
            body: JSON.stringify({a: a, b: b, operation: operation})
        });

        const data = await response.json();

        if (!response.ok) {
            result.textContent = "Ошибка: " + (data.error || "Ошибка сервера");
            return;
        }

        result.textContent = "Результат: " + data.result;
    } catch (error) {
        result.textContent = "Не удалось связаться с сервером";
        console.error(error);
    } finally { button.disabled = false; }
});