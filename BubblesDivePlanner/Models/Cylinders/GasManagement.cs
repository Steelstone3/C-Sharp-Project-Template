using BubblesDivePlanner.Controllers;

namespace BubblesDivePlanner.Models.Cylinders
{
    public class GasManagement : IGasManagement
    {
        private readonly ICylinderController cylinderController;

        public GasManagement(ICylinderController cylinderController, ushort initialPressurisedVolume, byte surfaceAirConsumptionRate)
        {
            this.cylinderController = cylinderController;
            RemainingGas = initialPressurisedVolume;
            SurfaceAirConsumptionRate = surfaceAirConsumptionRate;
        }

        public ushort RemainingGas { get; private set; }
        public byte SurfaceAirConsumptionRate { get; }
        public ushort GasUsed { get; private set; }

        public void UpdateGasUsage(IDiveStep diveStep)
        {
            GasUsed = cylinderController.CalculateGasUsage(SurfaceAirConsumptionRate, diveStep);
            RemainingGas = cylinderController.CalculateRemainingGas(RemainingGas, GasUsed);
        }
    }
}