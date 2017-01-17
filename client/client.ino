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

void respond_moisture(int seq, int plant_id) {
    int moisture_level = analogRead(plant_id);

    int response[RESPONSE_SIZE / sizeof(int)];
    memset(response, 0, RESPONSE_SIZE);

    response[0] = seq;
    response[1] = plant_id;
    response[2] = DATA_LEN_MOISTURE;
    response[3] = moisture_level;

    Serial.write((const uint8_t*)&response, RESPONSE_SIZE);
}

void respond_error(int seq, const char* text, int text_len) {
    int response[RESPONSE_SIZE / sizeof(int)];
    memset(response, 0, RESPONSE_SIZE);

    response[0] = seq;
    response[1] = -1;
    response[2] = text_len;
    memcpy(&response[3], text, text_len);

    Serial.write((const uint8_t*)&response, RESPONSE_SIZE);
}

void loop() {
    // Serial packets are comprised of a sequence number request type and a plant ID so we need at least 2 bytes per response
    if (Serial.available() >= 3) {
        int seq_number = Serial.read();
        int req_type = Serial.read();
        switch (req_type) {
            case REQ_MOISTURE:
            respond_moisture(seq_number, req_type);
            break;

            case REQ_TEMPERATURE:
            respond_error(seq_number, REQ_TYPE_NOT_IMPLEMENTED, sizeof(REQ_TYPE_NOT_IMPLEMENTED));
            break;

            default:
            respond_error(seq_number, REQ_TYPE_UNKNOWN, sizeof(REQ_TYPE_UNKNOWN));
        }
    }
}

