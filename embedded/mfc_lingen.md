# Sensorstation Modellflugverein Lingen 

Für den MFC Lingen soll ein drahtloser Sensor erstellt werden, der in der Lage ist folgende Informationen zu sammeln:

* Temperatur
* Luftfeuchtigkeit
* Windgeschwindigkeit
* Präsenzmelder (optional)

## Plattform

Als Plattform wird ein NodeMCU mit der [ESP8266-Home](https://esphome.io)-Umgebung bevorzugt.

## Energieversorgung

Die Energieversorgung ist durch eine autarke Photovoltaik- und Windenergieanlage gesichert. Trotzdem sollte der drahtlose Sensor möglichst wenig Energie konsumieren, da die Batteriespeicher nicht zu sehr belastet werden sollen.

Der _Deep Sleep Mode (DSM)_ kann im NodeMCU-Modul mit folgendem Code eingeleitet werden:

```c
ESP.deepSleep(double us);
```

Damit der DSM funktioniert, muss der GPIO-Pin 16 mit dem Reset-Pin (RST) verbunden werden. Der Pin 16 wird vom aktivierten Timer getriggert. 

## Sensorkonzepte

| Art | Sensortyp |
| --- | --------- |
| Temperatur | BME 280 |
| Luftfeuchtigkeit | BME 280 |
| Windgeschwindigkeit | Leistungsmessung |

Der [BME 280](https://www.reichelt.de/kombo-sensor-luftdruck-luftfeuchtigkeit-temp-bme-280-p159825.html?&nbc=1) soll in der I2C-Ausführung verwendet werden. Der BME 280 muss kalibriert werden, da die Montage in einem Gehäuse erfolgen soll. Das führt zu leicht veränderten Messwerten, gegenüber eine offenen Montage.

Der BME 280 wird von der Plattform unterstützt: <https://esphome.io/components/sensor/bme280.html> 

### Windgeschwindigkeit

Aufgrund der Tatsache, dass die Energie teilweise von einer Windenergieanlage erzeugt wird, ist es möglich die generierte Leistung von dem Steuergerät abzugreifen und mithilfe der Kennlinie die Windgeschwindigkeit zu ermitteln. 

__Für die erfolgreiche Ermittlung der Windgeschwindigkeit, muss die Kennlinie der Windanlage bekannt sein. Das ist noch nicht der Fall.__
