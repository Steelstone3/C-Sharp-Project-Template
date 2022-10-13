using BubblesDivePlanner.Controllers;

namespace BubblesDivePlanner.Models.Cylinders
{
    public class GasMixture : IGasMixture
    {
        public GasMixture(byte oxygen, byte helium)
        {
            Oxygen = oxygen;
            Helium = helium;
            CheckEdgeCases();
            CalculateNitrogen();
        }

        public byte Oxygen { get; private set; }
        public byte Helium { get; private set; }
        public byte Nitrogen { get; private set; }

        private void CheckEdgeCases()
        {
            if (Oxygen == 0)
            {
                Oxygen = 5;
            }
            else if (Oxygen >= 100 && Helium >= 100)
            {
                Oxygen = 100;
                Helium = 0;
            }

            if (Oxygen >= 100)
            {
                Oxygen = (byte)(100 - Helium);
            }
            if (Helium >= 100)
            {
                Helium = (byte)(100 - Oxygen);
            }
        }

        private void CalculateNitrogen() => Nitrogen = (byte)(100 - Oxygen - Helium);
    }
}