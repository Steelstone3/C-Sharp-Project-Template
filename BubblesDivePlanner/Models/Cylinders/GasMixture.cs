namespace BubblesDivePlanner.Models.Cylinders
{
    public class GasMixture : IGasMixture
    {
        public GasMixture(byte oxygen, byte helium)
        {
            Oxygen = oxygen;
            Helium = helium;

            AssignGasMixture();
        }

        public byte Oxygen { get; private set; }
        public byte Helium { get; private set; }
        public byte Nitrogen { get; private set; }

        private void AssignGasMixture()
        {
            switch (Oxygen)
            {
                case 0:
                    Oxygen = 5;
                    break;
                case >= 100 when Helium >= 100:
                    Oxygen = 100;
                    Helium = 0;
                    break;
            }

            Oxygen = AssignOxygen();
            Helium = AssignHelium();
            CalculateNitrogen();
        }

        private byte AssignOxygen() => Oxygen >= 100 ? (byte)(100 - Helium) : Oxygen;
        private byte AssignHelium() => Helium >= 100 ? (byte)(100 - Oxygen) : Helium;
        private void CalculateNitrogen() => Nitrogen = (byte)(100 - Oxygen - Helium);
    }
}