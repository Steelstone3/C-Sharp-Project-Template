namespace BubblesDivePlanner.Models.Cylinders
{
    public class GasMixture : IGasMixture
    {
        public GasMixture(byte oxygen, byte helium)
        {
            Oxygen = oxygen;
            Helium = helium;
            WithOxygen();
            WithHelium();
        }

        public byte Oxygen { get; private set; }
        public byte Helium { get; private set; }
        public byte Nitrogen { get; private set; }

        private void WithOxygen()
        {
            CheckEdgeCases();

            if (Oxygen >= 100)
            {
                Oxygen = (byte)(100 - Helium);
            }

            CalculateNitrogen();
        }

        private void WithHelium()
        {
            CheckEdgeCases();

            if (Helium >= 100)
            {
                Helium = (byte)(100 - Oxygen);
            }

            CalculateNitrogen();
        }

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