const GoBackArrow = ({ backUrl }) => {
    const element = document.querySelector("#go-back-arrow");

    element.addEventListener("click", () => {
        window.location.href = backUrl;
    });
}
