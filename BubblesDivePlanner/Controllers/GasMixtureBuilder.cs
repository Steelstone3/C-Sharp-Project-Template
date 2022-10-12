using BubblesDivePlanner.Models.Cylinders;

namespace BubblesDivePlanner.Controllers
{
    public class GasMixtureBuilder : IGasMixtureBuilder
    {
        private byte oxygen = 0;
        private byte helium = 0;
        private byte nitrogen = 0;

        public IGasMixture Create()
        {
            WithOxygen(oxygen);
            WithHelium(helium);

            return new GasMixture(oxygen, helium, nitrogen);
        }


        public void WithOxygen(byte oxygen)
        {
            this.oxygen = oxygen;

            CheckEdgeCases();

            if (this.oxygen >= 100)
            {
                this.oxygen = (byte)(100 - helium);
            }

            CalculateNitrogen();
        }

        public void WithHelium(byte helium)
        {
            this.helium = helium;

            CheckEdgeCases();

            if (helium >= 100)
            {
                this.helium = (byte)(100 - oxygen);
            }

            CalculateNitrogen();
        }

        private void CalculateNitrogen() => nitrogen = (byte)(100 - oxygen - helium);

        private void CheckEdgeCases()
        {
            if (oxygen == 0)
            {
                oxygen = 5;
            }
            else if (oxygen >= 100 && helium >= 100)
            {
                oxygen = 100;
                helium = 0;
            }
        }
    }
}