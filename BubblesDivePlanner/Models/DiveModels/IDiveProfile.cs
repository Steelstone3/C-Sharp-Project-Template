namespace BubblesDivePlanner.Models.DiveModels
{
    public interface IDiveProfile
    {
        double[] MaxSurfacePressures
        {
            get;
        }

        double[] TissuePressuresNitrogen
        {
            get;
        }

        double[] TissuePressuresHelium
        {
            get;
        }

        double[] TissuePressuresTotal
        {
            get;
        }

        double[] ToleratedAmbientPressures
        {
            get;
        }

        double[] AValues
        {
            get;
        }

        double[] BValues
        {
            get;
        }

        double[] CompartmentLoads
        {
            get;
        }

        double PressureOxygen
        {
            get;
        }

        double PressureHelium
        {
            get;
        }

        double PressureNitrogen
        {
            get;
        }
    }
}