using BubblesDivePlanner.Controllers;

namespace BubblesDivePlanner.Models.Cylinders
{
    public class Cylinder : ICylinder
    {
        public Cylinder(ICylinderController cylinderController, byte cylinderVolume, ushort cylinderPressure, IGasMixture gasMixture, byte surfaceAirConsumptionRate)
        {
            CylinderVolume = cylinderVolume;
            CylinderPressure = cylinderPressure;
            GasMixture = gasMixture;
            InitialPressurisedVolume = (ushort)(cylinderPressure * cylinderVolume);
            GasManagement = new GasManagement(cylinderController, InitialPressurisedVolume, surfaceAirConsumptionRate);
        }

        public byte CylinderVolume { get; }
        public ushort CylinderPressure { get; }
        public ushort InitialPressurisedVolume { get; private set; }
        public IGasMixture GasMixture { get; }
        public IGasManagement GasManagement { get; }
    }
}