#include <Wire.h>
#include <Encoder.h>

// Définir les encodeurs (broches à ajuster selon votre montage)
Encoder encoder1(2, 3);
Encoder encoder2(4, 5);
Encoder encoder3(6, 7);
Encoder encoder4(8, 9);

void setup() {
  Wire.begin(8); // Adresse I2C de l'Arduino
  Wire.onRequest(requestEvent);
}

void loop() {
  // Laisser l'Arduino gérer les encodeurs en arrière-plan
}

// Fonction appelée quand le Raspberry demande des données
void requestEvent() {
  long pos1 = encoder1.read();
  long pos2 = encoder2.read();
  long pos3 = encoder3.read();
  long pos4 = encoder4.read();
  
  // Envoyer les positions sous forme d'un tableau structuré
  Wire.write((byte*)&pos1, sizeof(pos1));
  Wire.write((byte*)&pos2, sizeof(pos2));
  Wire.write((byte*)&pos3, sizeof(pos3));
  Wire.write((byte*)&pos4, sizeof(pos4));
}
