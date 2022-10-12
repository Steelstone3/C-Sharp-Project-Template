using System.Runtime.InteropServices;
using BubblesDivePlanner.Models.Cylinders;
using Newtonsoft.Json.Bson;

namespace BubblesDivePlanner.Controllers
{
    public interface IGasMixtureBuilder
    {
        void WithOxygen(byte enteredOxygen);
        void WithHelium(byte enteredHelium);
        IGasMixture Create();
    }
}