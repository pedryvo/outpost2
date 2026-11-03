
export function speakTacticalAlert(text) {
    try {
        if (typeof window !== 'undefined' && 'speechSynthesis' in window) {
            window.speechSynthesis.cancel();
            const utter = new SpeechSynthesisUtterance(text);
            utter.rate = 1.05;
            utter.pitch = 0.90; // Authoritative sci-fi military tone
            utter.volume = 0.85;
            window.speechSynthesis.speak(utter);
        }
    } catch (e) {
        // Graceful fallback if audio permissions are restricted
    }
}

export function cancelSpeechAlerts() {
    try {
        if (typeof window !== 'undefined' && 'speechSynthesis' in window) {
            window.speechSynthesis.cancel();
        }
    } catch (e) {}
}
