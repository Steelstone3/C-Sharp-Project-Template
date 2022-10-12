using BubblesDivePlanner.Controllers;

namespace BubblesDivePlanner.Models.Cylinders
{
    public class GasMixture : IGasMixture
    {
        public GasMixture(IGasMixtureBuilder gasMixtureBuilder, byte oxygen, byte helium)
        {
            gasMixtureBuilder.WithOxygen(oxygen);
            gasMixtureBuilder.WithHelium(helium);
            var gasMixture = gasMixtureBuilder.Create();
            Oxygen = gasMixture.Oxygen;
            Helium = gasMixture.Helium;
            Nitrogen = gasMixture.Nitrogen;
        }

        public GasMixture(byte oxygen, byte helium, byte nitrogen)
        {
            Oxygen = oxygen;
            Helium = helium;
            Nitrogen = nitrogen;
        }

        public byte Oxygen { get; private set; }
        public byte Helium { get; private set; }
        public byte Nitrogen { get; private set; }
    }
}