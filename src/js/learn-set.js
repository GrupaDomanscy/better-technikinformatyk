const escape = (payload) => {
    return payload.toString().replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;').replaceAll('"', '&quot;').replaceAll("'", '&#039;');
}

const chooseQuestionContainer = document.querySelector("#choose-question-container");

const QuestionIdentifier = (idx) => {
    return `<div class="p-5 text-center text-3xl text-neutral-200 hover:bg-stone-800 rounded-xl cursor-pointer">${escape(idx)}</div>`;
}

const renderTemplate = (template) => {
    const div = document.createElement("div");
    div.innerHTML = template;

    return div.firstChild;
}

const contentContainer = document.querySelector("#content-container");

const switchQuestion = (questionNumber) => {
    contentContainer.innerHTML = "";
    contentContainer.appendChild(renderTemplate(QuestionContainer(questions[questionNumber])));
}

const chooseAnswer = (idx) => {
    currentQuestion++;
    switchQuestion(currentQuestion);
}

const QuestionContainer = ({ question, code, image, answers }) => {
    return `<div class="w-full max-w-lg sm:max-w-xl lg:max-w-3xl p-5 flex flex-col items-center justify-center gap-8 mx-auto">
    <p class="text-3xl text-neutral-300 font-semibold">${escape(question)}</p>

    ${code !== null ? `<code class="w-full rounded-xl bg-stone-800 p-5 overflow-y-scroll text-neutral-300">${code}</code>` : ""}
    ${image !== null ? `<img src="${image}" />` : ""}

    <div class="flex flex-col w-full gap-4">
        ${answers.map((answer, idx) => `<div onclick="chooseAnswer(${escape(idx)})" class="text-neutral-300 p-3 px-5 cursor-pointer hover:bg-blue-800 duration-300 rounded-xl bg-stone-800">${escape(answer)}</div>`).join("")}
    </div>
</div>`;
}

let currentQuestion = 0, questions, questionsCount;

const main = async () => {
    questionsCount = await window.__TAURI__.invoke('get_question_count_from_state');
    questions = await window.__TAURI__.invoke('get_all_questions_from_state');

    for (let i = 0; i < questionsCount; i++) {
        chooseQuestionContainer.appendChild(renderTemplate(QuestionIdentifier(i + 1)));
    }

    contentContainer.appendChild(renderTemplate(QuestionContainer(questions[currentQuestion])));
}

main();
