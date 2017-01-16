const int HYDROMETER_PIN = 0;

void setup() {
    Serial.begin(9600);
}

void loop() {
    int val = analogRead(HYDROMETER_PIN);
    Serial.write(val);
    delay(1000);
}
