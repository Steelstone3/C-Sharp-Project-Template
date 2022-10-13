namespace BubblesDivePlanner.Models.Cylinders
{
    public class Cylinder : ICylinder
    {
        public Cylinder(ushort cylinderVolume, ushort cylinderPressure, IGasMixture gasMixture, byte surfaceAirConsumptionRate)
        {
            CylinderVolume = AssignCylinderVolume(cylinderVolume);
            CylinderPressure = AssignCylinderPressure(cylinderPressure);
            InitialPressurisedVolume = (ushort)(CylinderVolume * CylinderPressure);
            RemainingGas = InitialPressurisedVolume;
            SurfaceAirConsumptionRate = AssignSurfaceAirConsumptionRate(surfaceAirConsumptionRate);
            GasMixture = gasMixture;
        }

        public ushort CylinderVolume { get; }
        public ushort CylinderPressure { get; }
        public ushort InitialPressurisedVolume { get; private set; }
        public ushort RemainingGas { get; private set; }
        public ushort UsedGas { get; private set; }
        public byte SurfaceAirConsumptionRate { get; }
        public IGasMixture GasMixture { get; }

        public void UpdateCylinderGasConsumption(IDiveStep diveStep)
        {
            UsedGas = (ushort)(((diveStep.Depth / 10) + 1) * diveStep.Time * SurfaceAirConsumptionRate);
            RemainingGas = UsedGas < RemainingGas ? (ushort)(RemainingGas - UsedGas) : (ushort)0;
        }

        private static ushort AssignCylinderVolume(ushort cylinderVolume) => cylinderVolume switch
        {
            > 30 => 30,
            < 3 => 3,
            _ => cylinderVolume
        };

        private static ushort AssignCylinderPressure(ushort cylinderPressure) => cylinderPressure switch
        {
            > 300 => 300,
            < 50 => 50,
            _ => cylinderPressure,
        };

        private static byte AssignSurfaceAirConsumptionRate(byte surfaceAirConsumptionRate) => surfaceAirConsumptionRate switch
        {
            > 30 => 30,
            < 3 => 3,
            _ => surfaceAirConsumptionRate
        };
    }
}