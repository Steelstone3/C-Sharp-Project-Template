namespace BubblesDivePlanner.Models.Cylinders
{
    public class GasMixture : IGasMixture
    {
        public GasMixture(byte oxygen, byte helium)
        {
            Oxygen = oxygen;
            Helium = helium;

            CheckEdgeCases();
            Oxygen = AssignOxygen();
            Helium = AssignHelium();
            CalculateNitrogen();
        }

        public byte Oxygen { get; private set; }
        public byte Helium { get; private set; }
        public byte Nitrogen { get; private set; }

        private byte AssignOxygen() => Oxygen >= 100 ? (byte)(100 - Helium) : Oxygen;
        private byte AssignHelium() => Helium >= 100 ? (byte)(100 - Oxygen) : Helium;
        private void CalculateNitrogen() => Nitrogen = (byte)(100 - Oxygen - Helium);
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
        }
    }
}