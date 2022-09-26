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
            if (Oxygen == 0)
            {
                Oxygen += 5;
            }

            if (Oxygen >= 100 && Helium >= 100)
            {
                Oxygen = 100;
                Helium = 0;
            }
            else if (Oxygen >= 100)
            {
                Oxygen = (byte)(100 - Helium);
            }
            else if (Helium >= 100)
            {
                Helium = (byte)(100 - Oxygen);
            }

            Nitrogen = (byte)(100 - Oxygen - Helium);
        }
    }
}