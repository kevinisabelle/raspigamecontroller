namespace HidReportMapCreator.Hid;

public static class HidUsagePageSimulation
{
    public const ushort Undefined = 0x00;
    public const ushort FlightSimulationDevice = 0x01;
    public const ushort AutomobileSimulationDevice = 0x02;
    public const ushort TankSimulationDevice = 0x03;
    public const ushort SpaceshipSimulationDevice = 0x04;
    public const ushort SubmarineSimulationDevice = 0x05;
    public const ushort SailingSimulationDevice = 0x06;
    public const ushort MotorcycleSimulationDevice = 0x07;
    public const ushort SportsSimulationDevice = 0x08;
    public const ushort AirplaneSimulationDevice = 0x09;
    public const ushort HelicopterSimulationDevice = 0x0A;
    public const ushort MagicCarpetSimulationDevice = 0x0B;
    public const ushort BicycleSimulationDevice = 0x0C;
    // 0x0D - 0x1F Reserved

    public const ushort FlightControlStick = 0x20;
    public const ushort FlightStick = 0x21;
    public const ushort CyclicControl = 0x22;
    public const ushort CyclicTrim = 0x23;
    public const ushort FlightYoke = 0x24;
    public const ushort TrackControl = 0x25;
    // 0x26 - 0xAF Reserved

    public const ushort Aileron = 0xB0;
    public const ushort AileronTrim = 0xB1;
    public const ushort AntiTorqueControl = 0xB2;
    public const ushort AutopilotEnable = 0xB3;
    public const ushort ChaffRelease = 0xB4;
    public const ushort CollectiveControl = 0xB5;
    public const ushort DiveBrake = 0xB6;
    public const ushort ElectronicCountermeasures = 0xB7;
    public const ushort Elevator = 0xB8;
    public const ushort ElevatorTrim = 0xB9;
    public const ushort Rudder = 0xBA;
    public const ushort Throttle = 0xBB;
    public const ushort FlightCommunications = 0xBC;
    public const ushort FlareRelease = 0xBD;
    public const ushort LandingGear = 0xBE;
    public const ushort ToeBrake = 0xBF;

    public const ushort Trigger = 0xC0;
    public const ushort WeaponsArm = 0xC1;
    public const ushort WeaponsSelect = 0xC2;
    public const ushort WingFlaps = 0xC3;
    public const ushort Accelerator = 0xC4;
    public const ushort Brake = 0xC5;
    public const ushort Clutch = 0xC6;
    public const ushort Shifter = 0xC7;
    public const ushort Steering = 0xC8;
    public const ushort TurretDirection = 0xC9;
    public const ushort BarrelElevation = 0xCA;
    public const ushort DivePlane = 0xCB;
    public const ushort Ballast = 0xCC;
    public const ushort BicycleCrank = 0xCD;
    public const ushort HandleBars = 0xCE;
    public const ushort FrontBrake = 0xCF;
    public const ushort RearBrake = 0xD0;
    // 0xD1 - 0xFFFF Reserved
}