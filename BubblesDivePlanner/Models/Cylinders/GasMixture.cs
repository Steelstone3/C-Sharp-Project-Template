using System.Net.Http;
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

            Oxygen = AssignOxygen();
            Helium = AssignHelium();
        }

        private byte AssignOxygen() => Oxygen >= 100 ? (byte)(100 - Helium) : Oxygen;
        private byte AssignHelium() => Helium >= 100 ? (byte)(100 - Oxygen) : Helium;
        private void CalculateNitrogen() => Nitrogen = (byte)(100 - Oxygen - Helium);
    }
}