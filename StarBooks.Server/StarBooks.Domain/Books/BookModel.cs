using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Diagnostics.CodeAnalysis;
using System.Text.Json.Serialization;

namespace StarBooks.Domain.Books;

[Table("books")]
public class BookModel
{
    [JsonIgnore, Key]
    public int Id { get; set; }

    [JsonPropertyName("volumeInfo")]
    public VolumeInfo VolumeInfo { get; set; }

    [JsonPropertyName("saleInfo")]
    public SaleInfo SaleInfo { get; set; }

    [JsonPropertyName("accessInfo")]
    public AccessInfo AccessInfo { get; set; }

    [JsonPropertyName("searchInfo")]
    public SearchInfo SearchInfo { get; set; }
    
    [JsonIgnore]
    public List<AuthorModel> Authors { get; set; }
    
    [JsonIgnore]
    public List<CategoryModel> Categories { get; set; }
    
    [JsonIgnore]
    public List<IndustryIdentifierModel> Identifiers { get; set; }
}
