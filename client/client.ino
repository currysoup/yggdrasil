const int REQ_MOISTURE = 0;
const int REQ_TEMPERATURE = 1;

// Fields:               seq_num     + plant_id    + len         + buf
const int RESPONSE_SIZE = sizeof(int) + sizeof(int) + sizeof(int) + 256;

// 1 int just for the moisture level
const int DATA_LEN_MOISTURE = 1;

const char* REQ_TYPE_NOT_IMPLEMENTED = "Request type not implemented";
const char* REQ_TYPE_UNKNOWN = "Request type unknown";

void setup() {
    Serial.begin(57600);
}

void respond(int seq, int plant_id, char* text) {
    char buffer[256];
    sprintf(buffer, "{\"seq\":\"%i\",\"plant_id\":\"%i\",\"text\":\"%s\"}", seq, plant_id, text);
    Serial.println(buffer);
    Serial.flush();
}

void respond_moisture(int seq, int plant_id) {
    int moisture_level = analogRead(plant_id);
    char buf[5];
    respond(seq, plant_id, itoa(moisture_level, buf, 10));
}

void loop() {
    while (Serial.available() > 0) {
        int seq_number = Serial.parseInt();

        int req_type = Serial.parseInt();

        int plant_id = Serial.parseInt();

        if (Serial.read() == '\n') {
            respond_moisture(seq_number, plant_id);
        }
    }
}

