#include <Wire.h>
#include <RotaryEncoder.h>
// #include <LiquidCrystal_I2C.h>

// Définir les encodeurs (broches à ajuster selon votre montage)
RotaryEncoder encoder1(2, 3, RotaryEncoder::LatchMode::FOUR3);
RotaryEncoder encoder2(4, 5, RotaryEncoder::LatchMode::FOUR3);
// RotaryEncoder encoder3(6, 7, RotaryEncoder::LatchMode::FOUR3);
// RotaryEncoder encoder4(8, 9, RotaryEncoder::LatchMode::FOUR3);

// Initialiser l'écran LCD I2C (adresse typique: 0x27 ou 0x3F)
// LiquidCrystal_I2C lcd(0x27, 16, 2); // Adresse 0x27, écran 16x2

long lastPos3 = 0; // Sauvegarde de la dernière valeur affichée

void setup() {
  Wire.begin(8); // Adresse I2C de l'Arduino
  Wire.onRequest(requestEvent);

  // Initialisation de l'écran LCD
  /*lcd.init();
  lcd.backlight();
  lcd.setCursor(0, 0);
  lcd.print("Enc3:");*/

  // Activer les pull-ups internes
  pinMode(2, INPUT_PULLUP);
  pinMode(3, INPUT_PULLUP);
  pinMode(4, INPUT_PULLUP);
  pinMode(5, INPUT_PULLUP);
  // pinMode(6, INPUT_PULLUP);
  // pinMode(7, INPUT_PULLUP);
  // pinMode(8, INPUT_PULLUP);
  // pinMode(9, INPUT_PULLUP);
}

void loop() {
  // Mettre à jour tous les encodeurs
  encoder1.tick();
  encoder2.tick();
  // encoder3.tick();
  // encoder4.tick();

  // Lire la position actuelle de l'encodeur 3
  /*long pos3 = encoder3.getPosition();

  // Mettre à jour l'écran LCD uniquement si la valeur change
  if (pos3 != lastPos3) {
    lastPos3 = pos3;
    lcd.setCursor(5, 0); // Placer le curseur après "Enc3:"
    lcd.print("      "); // Effacer ancienne valeur
    lcd.setCursor(5, 0);
    lcd.print(pos3);
  }*/
}

// Fonction appelée quand le Raspberry demande des données
void requestEvent() {

  long pos1 = encoder1.getPosition();
  long pos2 = encoder2.getPosition();
  // long pos3 = encoder3.getPosition();
  // long pos4 = encoder4.getPosition();
  uint8_t wrappedPos1 = (uint8_t)(encoder1.getPosition() & 0xFF);
  uint8_t wrappedPos2 = (uint8_t)(encoder2.getPosition() & 0xFF);
  Wire.write(wrappedPos1);
  Wire.write(wrappedPos2);
  // Wire.write((uint8_t*)&pos3, sizeof(long));
  // Wire.write((uint8_t*)&pos4, sizeof(long));
}
